export class HandlebarsInput {
  constructor(element) {
    this.element = element
    element.oninput = this.oninput.bind(this)
    element.onkeydown = this.onkeydown.bind(this)
  }

  setTemplate(json) {
    this.template = json
  }

  oninput(evt) {
    this.checkIfAutoCompleteShouldTrigger()
  }

  checkIfAutoCompleteShouldTrigger() {
    const location = this.element.selectionStart
    const openingBrackets = this.element.value.lastIndexOf("{{")
    const closingBrackets = this.element.value.lastIndexOf("}}")
    if (openingBrackets !== -1 && openingBrackets > closingBrackets) {
      // We're in a place where we should show auto-complete
      let afterBrackets = this.element.value.slice(openingBrackets + 2, this.element.selectionStart)
      const completions = this.completions(afterBrackets)
      if (completions.length > 0) {
        this.showAutocomplete(completions)
        return
      }
    }
    this.hideAutocomplete()
  }

  completions(after) {
    const completions = (template, prefix) => Object.entries(template)
      .filter(([key]) => key.startsWith(prefix))
      .filter(([_, value]) => value != null)
      .map(([key, value]) => {
        let withoutPrefix = key.slice(prefix.length)
        if (typeof value === "object") {
          return {key, completion: `${withoutPrefix}.`}
        } else {
          return {key, value, completion: `${withoutPrefix}}}`}
        }
      })
    if (after !== undefined && after.length > 0) {
      // Return all keys under this path
      const path = after.split('.')//.concat([""])
      return path.reduce((template, path) => {
        if (template[path] !== undefined) {
          return template[path]
        } else {
          return completions(template, path)
        }
      }, this.template)
    } else {
      // Return all top-level keys
      return completions(this.template, "")
    }
  }

  showAutocomplete(completions) {
    this.hideAutocomplete()

    const container = document.createElement("table")
    container.classList.add("autocomplete")
    completions.map(({key, value, completion}) => {
      let eln = document.createElement("tr")
      let keyEln = document.createElement("td")
      keyEln.textContent = key
      eln.appendChild(keyEln)
      let valueEln = document.createElement("td")
      valueEln.textContent = value
      eln.appendChild(valueEln)
      eln.dataset.completion = completion
      eln.onclick = () => this.completeWith(eln)
      return eln
    }).forEach(eln => container.appendChild(eln))

    this.autocomplete = container
    this.element.parentNode.insertBefore(container, this.element.nextSibling)
  }

  hideAutocomplete() {
    if (this.autocomplete) {
      this.element.parentNode.removeChild(this.autocomplete)
      this.autocomplete = undefined
    }
  }

  onkeydown(evt) {
    if (this.autocomplete === undefined) {
      return
    }

    const active = this.autocomplete.getElementsByClassName("active")[0]
    let next = undefined
    if (evt.keyCode == 40) {
      next = active?.nextSibling ?? this.autocomplete.children[0]
    } else if (evt.keyCode == 38) {
      next = active?.previousSibling ?? this.autocomplete.children[this.autocomplete.children.length - 1]
    } else if (evt.keyCode == 13) {
      this.completeWith(active)
    } else {
      return
    }
    evt.preventDefault()

    if (next !== undefined) {
      next.classList.add("active")
      active?.classList.remove("active")
    }
  }

  completeWith(eln) {
    const completion = eln.dataset.completion
    this.element.value = this.element.value.substring(0, this.element.selectionStart) + completion + this.element.value.substring(this.element.selectionEnd, this.element.value.length)
    this.element.focus()
    const caretPosition = this.element.selectionStart + completion.length
    this.element.setSelectionRange(caretPosition, caretPosition)
    this.checkIfAutoCompleteShouldTrigger()
  }
}


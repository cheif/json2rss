import { HandlebarsInput } from "./handlebars-input.js"

const render = (rss) => {
  const data = new window.DOMParser().parseFromString(rss, "text/xml")
  let html = `
    <h1>
      <a href="${data.querySelector("link").innerHTML}">
        ${data.querySelector("title").innerHTML}
      </a>
    </h1>`
  data.querySelectorAll("item").forEach(el => {
    let description = ""
    el.querySelector("description").childNodes.forEach(n => description += n.textContent)
    html += `<article>
      <h2>
        <a href="${el.querySelector("guid").innerHTML}">
          ${el.querySelector("title").innerHTML}
        </a>
      </h2>
      ${description.replace(/\n/g, "<br/>")}
    </article>`
  })
  const element = document.getElementById("rss")
  element.innerHTML = html
}

class App {
  constructor(form, rssElement) {
    this.form = form
    this.rssElement = rssElement
    this.form.onsubmit = this.processForm.bind(this)
    this.form.source.oninput = this.sourceChanged.bind(this)
    this.description_input = new HandlebarsInput(form.description_template)
  }

  setTemplate(json) {
    this.description_input.setTemplate(json[this.form.item_key.value][0])
  }

  sourceChanged() {
    fetch(this.form.source.value).then(res => res.json()).then(this.setTemplate.bind(this))
  }

  processForm() {
    const config = {
      source: this.form.source.value,
      link: this.form.link.value,
      title: this.form.title.value,
      item_key: this.form.item_key.value,
      url_template: this.form.url_template.value,
      title_template: this.form.title_template.value,
      description_template: this.form.description_template.value
    }

    const url = `/feed?config=${encodeURIComponent(JSON.stringify(config))}`
    fetch(url)
      .then(res => res.text())
      .then(render)
    return false;
  }
}

window.app = new App(document.forms[0], document.getElementById("rss"));

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
const processForm = (evt) => {
  const form = evt.target
  const config = {
    source: form.source.value,
    link: form.link.value,
    title: form.title.value,
    item_key: form.item_key.value,
    url_template: form.url_template.value,
    title_template: form.title_template.value,
    description_template: form.description_template.value
  }

  const url = `/feed?config=${encodeURIComponent(JSON.stringify(config))}`
  fetch(url)
    .then(res => res.text())
    .then(render)
  return false;
}

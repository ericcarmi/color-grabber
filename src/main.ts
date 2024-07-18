import { listen } from '@tauri-apps/api/event'

const unlisten = listen("color", (event) => {
  let rgb = event.payload as number[]
  let r = rgb[0]
  let g = rgb[1]
  let b = rgb[2]
  let e = document.getElementById("root")
  if (e) {
    e.style.background = `rgb(${r}, ${g}, ${b})`
    e.style.color = `rgb(${255-r}, ${255-g}, ${255-b})`
    e.innerHTML=`
    <span>${r}, ${g}, ${b}<span>
    <br/>
    <span>${r.toString(16)}${g.toString(16)}${b.toString(16)}<span>
    `
  }
  // console.log(rgb)

})

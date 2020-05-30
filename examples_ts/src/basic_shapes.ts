import scenelib from '../../pkg/scenelib.js'

let scene = new scenelib.Scene()
scene.add_poly(scenelib.square())

let plot = scene.to_plot()

let path = scenelib.to_svg_path(plot)
console.log(path)

import { GLandCA, play_sine_wave } from './pkg';
import 'bootstrap/dist/css/bootstrap.min.css';
import * as seedrandom from 'seedrandom';

const inputForm = <HTMLInputElement>document.getElementById('inputForm');
const canvasContainer = document.getElementById('canvasContainer');
const canvas = <HTMLCanvasElement>document.getElementById('canvas');
const numCellTypesInput = <HTMLInputElement>document.getElementById('numCellTypes');
const universeWidthInput = <HTMLInputElement>document.getElementById('universeWidth');
const ruleDensityInput = <HTMLInputElement>document.getElementById('ruleDensity');
const ruleDensityOutput = <HTMLInputElement>document.getElementById('ruleDensityOutput');
const seedInput = <HTMLInputElement>document.getElementById('seed');
const seedOption = <HTMLInputElement>document.getElementById('seedOption');
const animateOption = <HTMLInputElement>document.getElementById('animateOption');
const animationSpeed = <HTMLInputElement>document.getElementById('animationSpeed');
const animationOutput = <HTMLInputElement>document.getElementById('animationSpeedOutput');
const audioButton = <HTMLButtonElement>document.getElementById('audioButton');

let frameNum = 0;
let lastFrame = performance.now();

ruleDensityOutput.value = ruleDensityInput.value;
animationOutput.value = animationSpeed.value;
animationSpeed.disabled = !animateOption.checked;
ruleDensityInput.oninput = () => { ruleDensityOutput.value = ruleDensityInput.value; }
animationSpeed.oninput = () => { animationOutput.value = animationSpeed.value; }
animateOption.oninput = () => { animationSpeed.disabled = !animateOption.checked; }
seedOption.oninput = () => { seedInput.disabled = seedOption.checked; }

function generateAndRenderCA() {
  if (seedOption.checked) {
    seedInput.value = window.btoa(String(Math.floor(Math.random() * 4294967296)));
  }
  seedrandom(seedInput.value, { global: true });
  canvasContainer.style.display = 'block';
  cancelAnimationFrame(frameNum);

  let gl_ca = new GLandCA(
    canvas,
    Number(numCellTypesInput.value),
    Number(universeWidthInput.value),
    Number(ruleDensityInput.value),
  );

  if (animateOption.checked) {
    let renderLoop = (timestamp: DOMHighResTimeStamp) => {
      if ((timestamp - lastFrame) > (1000 / Number(animationSpeed.value))) {
        gl_ca.draw_animation_frame();
        lastFrame = timestamp;
      }
      frameNum = window.requestAnimationFrame(renderLoop);
    }

    frameNum = window.requestAnimationFrame(renderLoop);
  } else {
    gl_ca.draw_entire_ca();
  }
}

inputForm.addEventListener('submit', (event) => {
  if (inputForm.checkValidity()) {
    generateAndRenderCA();
  } else {
    event.preventDefault()
    event.stopPropagation()
  }
  inputForm.classList.add('was-validated')
}, false)

audioButton.addEventListener('click', () => {
  play_sine_wave()
})

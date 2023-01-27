import { render } from './pkg';
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

ruleDensityInput.oninput = () => { ruleDensityOutput.value = ruleDensityInput.value; }
seedOption.oninput = () => { seedInput.disabled = seedOption.checked; }

inputForm.addEventListener('submit', (event) => {
  if (!inputForm.checkValidity()) {
    event.preventDefault()
    event.stopPropagation()
  } else {
    if (seedOption.checked) {
      seedInput.value = window.btoa(String(Math.floor(Math.random() * 4294967296)));
    }
    seedrandom(seedInput.value, { global: true });
    canvasContainer.style.display = 'block';
    cancelAnimationFrame(requestAnimationFrame(() => { }) - 1);
    render_js();
  }

  inputForm.classList.add('was-validated')
}, false)

function render_js() {
  render(
    canvas,
    Number(numCellTypesInput.value),
    Number(universeWidthInput.value),
    Number(ruleDensityInput.value));
}

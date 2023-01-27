import { generateCa } from './pkg';
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

ruleDensityOutput.value = ruleDensityInput.value;
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

    generateCa(
      canvas,
      Number(numCellTypesInput.value),
      Number(universeWidthInput.value),
      Number(ruleDensityInput.value),
      animateOption.checked);
  }

  inputForm.classList.add('was-validated')
}, false)

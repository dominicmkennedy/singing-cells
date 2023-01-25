import { render, get_gl_context, get_program } from './pkg';
import 'bootstrap/dist/css/bootstrap.min.css';
const seedrandom = require('seedrandom');

const gl = get_gl_context();
const program = get_program(gl);

const inputForm = document.getElementById('inputForm');
const canvasContainer = document.getElementById('canvasContainer');
const numCellTypesInput = document.getElementById('numCellTypes');
const universeWidthInput = document.getElementById('universeWidth');
const ruleDensityInput = document.getElementById('ruleDensity');
const ruleDensityOutput = document.getElementById('ruleDensityOutput');
const seedInput = document.getElementById('seed');
const seedOption = document.getElementById('seedOption');

ruleDensityInput.oninput = () => { ruleDensityOutput.value = ruleDensityInput.value; }
seedOption.oninput = () => { seedInput.disabled = seedOption.checked; }

inputForm.addEventListener('submit', (event) => {
  if (!inputForm.checkValidity()) {
    event.preventDefault()
    event.stopPropagation()
  } else {
    if (seedOption.checked) {
      seedInput.value = window.btoa(Math.floor(Math.random() * 4294967296))
    }
    seedrandom(seedInput.value, { global: true });
    canvasContainer.style.display = 'block';
    render_js();
  }

  inputForm.classList.add('was-validated')
}, false)

function render_js() {
  render(
    gl,
    program,
    Number(numCellTypesInput.value),
    Number(universeWidthInput.value),
    Number(ruleDensityInput.value));
}

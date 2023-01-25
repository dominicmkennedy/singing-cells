import { render, get_gl_context, get_program } from './pkg';
import 'bootstrap/dist/css/bootstrap.min.css';

const gl = get_gl_context();
const program = get_program(gl);

const inputForm = document.getElementById('inputForm');
const canvasContainer = document.getElementById('canvasContainer');
const numCellTypesInput = document.getElementById('numCellTypes');
const universeWidthInput = document.getElementById('universeWidth');
const ruleDensityInput = document.getElementById('ruleDensity');
const ruleDensityOutput = document.getElementById('ruleDensityOutput');

ruleDensityInput.oninput = function() {
  ruleDensityOutput.value = ruleDensityInput.value;
}

inputForm.addEventListener('submit', function(event) {
  if (!inputForm.checkValidity()) {
    event.preventDefault()
    event.stopPropagation()
  } else {
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

import { render, get_gl_context, get_program } from './pkg';

const gl = get_gl_context();
const program = get_program(gl);

const renderButton = document.getElementById('renderButton');
const numCellTypesInput = document.getElementById('numCellTypes');
const universeWidthInput = document.getElementById('universeWidth');
const maxTimeStepsInput = document.getElementById('maxTimeSteps');
const ruleDensityInput = document.getElementById('ruleDensity');

function render_js() {
  let numCellTypes = Number(numCellTypesInput.value);
  let universeWidth = Number(universeWidthInput.value);
  let maxTimeSteps = Number(maxTimeStepsInput.value);
  let ruleDensity = Number(ruleDensityInput.value);

  render(gl, program, numCellTypes, universeWidth, maxTimeSteps, ruleDensity);
}

renderButton.addEventListener('click', render_js);

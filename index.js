const renderButton = document.getElementById('renderButton');
const numCellTypesInput = document.getElementById('numCellTypes');
const universeWidthInput = document.getElementById('universeWidth');
const maxTimeStepsInput = document.getElementById('maxTimeSteps');
const ruleDensityInput = document.getElementById('ruleDensity');

import { start } from './pkg';

function render() {
  let numCellTypes = Number(numCellTypesInput.value);
  let universeWidth = Number(universeWidthInput.value);
  let maxTimeSteps = Number(maxTimeStepsInput.value);
  let ruleDensity = Number(ruleDensityInput.value);

  start(numCellTypes, universeWidth, maxTimeSteps, ruleDensity);
}

renderButton.addEventListener('click', render);

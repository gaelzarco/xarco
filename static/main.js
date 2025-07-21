/* 
 * /static/main.js
*/

import * as THREE from 'three';

document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll(".content")
    .forEach(el => el.classList.remove("fade_in"));
});

const scene = new THREE.Scene();

console.log(scene);

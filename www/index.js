import { count_days } from "how-many-days-haa";

const daysEl = document.getElementById("days");
daysEl.innerText = `${count_days()} days`;

const fadeEl = document.getElementById("fade");

daysEl.addEventListener('click', function() {
  fadeEl.classList.add("opacity-100");
}, false);

daysEl.addEventListener('mouseout', function() {
  fadeEl.classList.remove("opacity-100");
}, false);

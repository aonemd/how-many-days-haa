import { count_days } from "how-many-days-huh";

const daysEl = document.getElementById("days");
daysEl.innerText = `${count_days()} days left`;

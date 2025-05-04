const popup = document.getElementById("popup");
const showBtn = document.getElementById("showPopup");
const closeBtn = document.getElementById("closePopup");

// Show popup when button is clicked
showBtn.addEventListener("click", () => {
  popup.classList.remove("hidden");
});

// Close popup when close button is clicked
closeBtn.addEventListener("click", () => {
  popup.classList.add("hidden");
});

// Optional: Automatically show the popup on page load
// window.addEventListener("load", () => {
//   popup.classList.remove("hidden");
// });

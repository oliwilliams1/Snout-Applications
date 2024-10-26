const locations = [
  { 0: "https://oliwilliams1.github.io/Snout-Apps-Website/src/todo/" }, 
  { 1: "https://oliwilliams1.github.io/Snout-Apps-Website/src/chatbot/" }, 
  { 2: "https://google.com" }, 
  { 3: "https://google.com" }, 
  { 4: "https://google.com" }, 
  { 5: "https://google.com" }, 
  { 6: "https://google.com" }, 
  { 7: "https://google.com" }
];

document.addEventListener('DOMContentLoaded', () => {
  for (let i = 0; i < locations.length; i++) {
    const segment = document.getElementById(`seg${i + 1}`);

    segment.addEventListener('click', () => {
      window.location.href = locations[i][i];
    });
  }
});
document.getElementById('memeForm').addEventListener('submit', async function(event) {
    event.preventDefault(); // Prevent default form submission
  
    const formData = new FormData(this); // Create FormData object from form
  
    try {
      const response = await fetch('/generateMeme', {
        method: 'POST',
        body: formData
      });
  
      if (!response.ok) { // Check for successful response
        throw new Error(`HTTP error: ${response.status}`);
      }
  
      const memeData = await response.json();
      const memeContainer = document.getElementById('memeContainer');
      memeContainer.innerHTML = `<img src="${memeData.url}" alt="Generated Meme">`;
    } catch (error) {
      console.error('Error generating meme:', error);
      // Optionally display an error message to the user
    }
  });
  
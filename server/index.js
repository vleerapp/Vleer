const express = require('express');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
app.use(cors());
app.use(express.json());

app.post('/api/copy-avatar', (req, res) => {
  const sourceFile = req.body.source;
  const destinationFile = req.body.destination;

  // Ensure the destination directory exists
  const destinationDir = path.dirname(destinationFile);
  if (!fs.existsSync(destinationDir)) {
    fs.mkdirSync(destinationDir, { recursive: true });
  }

  // Copy the file
  fs.copyFileSync(sourceFile, destinationFile);

  res.status(200).send({ message: 'File copied successfully' });
});

const PORT = process.env.PORT || 3001;
app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});

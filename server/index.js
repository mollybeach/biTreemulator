const express = require('express');
const app = express();
const PORT = 8080;
const cors = require("cors");
const home = require("./routes/home");
const simulator = require("./routes/simulator");
const uploadfile= require("./routes/uploadfile");



app.use(cors());
app.use(express.json());
app.use("/", home);
app.use("/", simulator);
app.use("/", uploadfile);





app.listen(PORT, () => {
   console.log(`Server is running on port ${PORT}`);
});
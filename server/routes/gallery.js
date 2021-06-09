const express = require("express");
const treeList = require("../data/tree.json");
const router = express.Router();
const fs = require("fs");

router.get("/gallery", (req, res) => {
   res.status(200).json(treeList);
});

module.exports = router;
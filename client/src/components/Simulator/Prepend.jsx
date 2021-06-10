/* eslint-disable */
import {binaryTree} from './binaryTree'
let Prepend = (inputtree) =>{
    let res= `let tree = JSON.parse(\`${inputtree}\`);\n let binaryTree = ${binaryTree} \n binaryTree(tree)`
    return res
   }

   export default Prepend
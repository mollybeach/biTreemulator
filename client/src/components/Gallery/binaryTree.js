import {tree} from '../Input/Input';
/* eslint-disable */
export function binaryTree() { 



let gen = getSpace(), sig=0;

function graph(gen) {
     displace(0.0,0.5,0.0);
    lightDirection(0.0, 1.0, 1.0);
    displace(0.16*gen.x, 0.16*gen.y, abs(abs(gen.y)-abs(gen.x))*0.025*sig);
    color(1.0,1.0,1.0)
    let pos1 = vec3(0.0, 0.0, 0.0), pos2 = vec3( 0.16*sig, 0.16, abs(abs(gen.y)-abs(gen.x))*0.025*-sig), ln=line(pos1, pos2, 0.01);
    color(0,0.0-(gen.y*0.1),0.9-sig)
    sphere(0.075);
    reset();
}

function build(tree, gen) {
    graph(gen)
    gen=gen-1;
   if (!tree.left && !tree.right) gen.y=gen.y+1;
   if (tree.left) sig=1,  build(tree.left, gen);
   if (tree.right) sig=-1, gen.x=gen.x+2, build(tree.right, gen);
  
}

  build(JSON.stringify(tree), gen);



}






  



/* eslint-disable */
export function binaryTree() { 

  let tree ={
    val: 0,
      left: 	{
        val: 1,
             left: { val: 2, 
                        left: { val: 3, 
                                   left: { val: 4, 
                                              left:{ val: 5, left:null, right:null},
                                              right: { val: 6, 
                                                          left:{ val: 7, left:null, right:null},
                                                          right:{ val: 8, left:null, right:null}
                                                        }, 
                                          }          
                              },
                       right: null
                   },
            right: { val: 9, 
                        left: { val: 10, left:null, right:null},
                        right: null
                } 
            },
    
     right: {
         val: 11,
               left: { val: 12, 
                        left: null, 
                        right: null
                   },
              right: {
                   val: 13,
                        left: null,
                        right: { val: 14, 
                                   left: { val: 15, 
                                              left:{ val: 16, 
                                                        left:null, 
                                                        right: { val: 17, left:null, right:null},
                                                    }, 
                                              right:null 
                                          }, 
                                   right: { val: 18, 
                                              left:null, 
                                              right: { val: 19, 
                                                          left:{ val: 20, left:null, right:null},
                                                          right:null
                                                        }
                                          },
               }
             }
           }
  }


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
  
    build(tree, gen);
  console.log(tree)
  
  
  }

  
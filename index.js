const rust = import("./pkg/rust_3d");
const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext('webgl', { antialias: true});

rust.then(m => {
  if(!gl){
    alert("Failed to initialize WebGL");
    return;
  }

  const FPS_THROTTLE = 1000.0 / 30.0; // miliseconds / frames
  const myClient = new m.MyClient();
  const initialTime = Date.now();
  var lastDrawTime = -1; // in miliseconds

  function render(){
    window.requestAnimationFrame(render);
    const currTime = Date.now();

    // check if its the time to update
    if(currTime >= lastDrawTime + FPS_THROTTLE){
      lastDrawTime = currTime;

      // also make sure when browser resizes we need to 
      // resize the animation as well
      if( window.innerHeight != canvas.height || window.innerWidth != canvas.width ){
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0,0, window.innerWidth, window.innerHeight);
      }

      let elapsedTime = currTime - initialTime;
      myClient.update(elapsedTime, window.innerHeight, window.innerWidth);
      myClient.render();
    }
  }
  render();
})

const formbtn = document.querySelector(".play-button");
const formwrapper = document.querySelector(".world");
const form_data=document.querySelector(".input_data");
let flag = false;

form_data.addEventListener('submit',(e)=>{
    e.preventDefault();
})

formbtn.addEventListener("click", function () {
  if (flag == false) {
    formwrapper.style.display = "contents";
    flag = true;
  }
  else{
        formwrapper.style.display="none";
        flag=false;
  }
});

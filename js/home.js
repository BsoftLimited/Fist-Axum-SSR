document.addEventListener("alpine:init", ()=>{
    Alpine.store(
        "user", {
            loading: false,
            value: undefined,
            login(){
                this.value = true;
                fetch("http://localhost:3000/api/user").then(response => response.json()).then(data =>{
                    this.value = false;
                    console.log(data);
                    this.value = data;
                });
            },
            logout(){
                this.value = undefined;
            },
            signup(name, surname, email, password, repassword){
                if(!name.length){
                    alert("Name field cannot be empty");
                }else if(!surname.length){
                    alert("Surname field cannot be empty");
                }else if(!email.length){
                    alert("Email field cannot be empty");
                }else if(!password.length){
                    alert("Passowrd field cannot be empty");
                }else if(password !== repassword){
                    alert("Passowrd Mismatch");
                }else{
                    this.value = true;
                    const details = { name: name,  surname: surname, email: email, password: password };
                    const options = { method: 'POST', headers: { 'Content-Type': 'application/json', }, body: JSON.stringify(details), };
                    fetch('https://jsonplaceholder.typicode.com/posts', options).then(response => response.json()).then(data =>{
                        this.value = false;
                        console.log(data);
                        this.value = data;
                    });
                }
            }
        }
    );
});
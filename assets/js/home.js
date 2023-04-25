document.addEventListener("alpine:init", ()=>{
    Alpine.store(
        "user", {
            loading: false,
            value: undefined,
            login(email, password){
                this.loading = true;
                const details = { email: email, password: password };
                    const options = { method: 'POST', headers: { 'Content-Type': 'application/json', }, body: JSON.stringify(details), };
                fetch("/api/user/login", options).then(response => response.json()).then(data =>{
                    this.loading = false;
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
                    this.loading = true;
                    const details = { name: name,  surname: surname, email: email, password: password };
                    const options = { method: 'POST', headers: { 'Content-Type': 'application/json', }, body: JSON.stringify(details), };
                    fetch('/api/user', options).then(response => response.json()).then(data =>{
                        this.loading = false;
                        console.log(data);
                        this.value = data;
                    });
                }
            }
        }
    );
});
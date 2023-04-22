document.addEventListener("alpine:init", ()=>{
    Alpine.store(
        "users", {
            loading: false,
            value: undefined,
            init(){
                this.value = true;
                fetch("/api/user/all").then(response => response.json()).then(data =>{
                    this.value = false;
                    console.log(data);
                    this.value = data;
                });
            },
        })
    }
);
const MAX: usize = 100;

struct Vectores {
    //atributos 

    dimension: usize,
    elemento : [u64 ; MAX],

}
impl Vectores{

    //constructor 

    fn new() -> Vectores{
        Vectores{
            dimension :0, 
            elemento : [0 ;MAX],
        }
        
    }
     ///saber cuantos elementos hay
    fn dim(&self)-> usize{   
        self.dimension
    }
    ///cambiar dimension, osea q agrega un 0 para q la "longitud" aumente 
    fn dimensionar(&mut self, d: usize){ 
        self.dimension=d; // tenias 8 de tamaño y quieres q sea d=11
    }
    // agregar al final 
    fn addelement( &mut self, e:u64){ 
        if self.dimension < MAX{  // prgunta si el tamaña es menor al maximo 8<100
            self.elemento[self.dimension]=e;  //agrega el numero
            self.dimension +=1;                 //agranda la dimension 
        }

    } //muestra cuantos hay
    fn mostrar(&self ){ 
        for i in 0.. self.dimension{   //recorre los elemntos por posicion de 0 asta el tamaño
            println!("Elemento [{}] = {}", i, self.elemento[i]);   //muestra cada uno por suu posicion
        }

    } //cambiar un elemnto 
    fn reemplazar(&mut self, p : usize, e :u64){  //instroducimos la posicion de 1.... y el elemento "11"
        if p<= self.dimension{                    //pregunta si la posicion es meor o igual q la dimension por q esta en vista del usuario
            self.elemento[p-1]=e;               // introduce el elemento en la posicion 
                                                 // p-1 = porq la maquina trabaja desde 0

        }
    }
    //desplaza los elemntos para poder insertar 
    fn insertar(&mut self, p : usize, e : u64){
        if (self.dimension < MAX)&& (p<= self.dimension){   //prgunta si en la dimension todavia se puede añadir Y si la posicion es menor = a la dimension
            self.dimension +=1;  //hace campo
            let mut x = self.dimension;  //crea una copia
            while x> p-1{                       // pregunta q mientras x sea mayo q la psocion "de la maquina 0..."
                self.elemento[x-1]= self.elemento[x-2];    //hace q el elemento de la derecha cope la izquierda 
                x -=1;                                      //el puntero reduce la posicion pa la siguiente vuelta
            }
            self.elemento[p-1]=e;                   //una ves q se llega a la posicion se cambia el elemento 
        }
    }  
    //devolver la posicion del del numero 
    fn buscar(&self ,  e: u64 )-> usize{
        let mut c :usize=0;
        for i in 0.. self.dimension {
                if self.elemento[i] ==e{
                    c=i;
                    
                     }

            }
             c
    }
    //metodo para eliminar un nro del vector por posicion 
    fn elimina(&mut self, e: u64){

        let pos = self.buscar(e); 
        for i in pos..self.dimension-1 {

            self.elemento[i] = self.elemento[i+1]

            
        }
        self.dimension -=1;

    }
    //metodo pa investir vector
    fn investir(&mut self){


        let mut iz = 0;
        let mut dere= self.dimension-1;
             
        while iz< dere{
              let mut temp= self.elemento[dere];


            self.elemento[iz]= self.elemento[dere];
            self.elemento[dere]= temp;
            iz +=1;
            dere +=1;
        }
    }

        //metodo para ordenar un vector de manera acendente
        fn ordenar(&mut self){

            for i in 0..self.dimension {
                
                for j in (i+1)..self.dimension{
                    if self.elemento[i]> self.elemento [j]{
                        let mut x = self.elemento[i];
                        self.elemento [i]= self.elemento[j];
                        self.elemento[j]=x;
                    }
                }
            }
     
    }
    //metodo para eliminar todos los numeros impares 
    fn elimina_par(&mut self){


            for i in 0..self.dimension {

                if self.elemento[i] % 2 == 0 {

                    self.elimina(self.elemento[i]);

                   
                }
                
            }

    }
    fn elimina_impar(&mut self){

        let mut i : usize=0;

        while i< self.dimension{
            if self.elemento[i] % 2 !=0{
                let n: u64 = self.elemento[i];
                self.elimina(n);
            }
            else {
                i +=1;
            }
        }
    }
     fn ordenar_impares_pares(&mut self){
        if self.dimension > 1{
            for i in 0..self.dimension {
                for j in 0..(self.dimension-1-i) {
                    let a = self.elemento[j];
                    let b = self.elemento[j+1];
                    
                    let a_espar = a % 2 == 0;
                    let b_espar = b % 2 ==0;
                    let mut intercambiar = false;
                    if !a_espar && !b_espar{
                        if a > b {
                            intercambiar = true;
                        }
                    }else if a_espar && b_espar{
                        if a > b {
                            intercambiar = true;
                        }
                    }else if a_espar && !b_espar{
                        intercambiar = true;
                    }else{
                        intercambiar = false;
                    }
                    if intercambiar {
                        self.elemento[j] = b;
                        self.elemento[j+1] = a;
                    }

                }
            }
        }
    }
    fn ordenar_burbuja(&mut self){
        if self.dimension > 1{
            for i in 0..self.dimension {
                for j in 0..(self.dimension-1-i) {
                    if self.elemento[j] > self.elemento[j+1]{
                        let aux = self.elemento[j];
                        self.elemento[j] = self.elemento[j+1];
                        self.elemento[j+1] = aux;
                    }
                }
            }
        }
    }
     fn ordenar_adentro_hacia_afuera(&mut self){
        if self.dimension > 1{
            self.ordenar_burbuja();

            let mut temp = [0; MAX];
            let pos_central= self.dimension / 2;
            let mut izq = 1;
            let mut der = 1;
            for i in 0..self.dimension {
                if i == 0{
                    temp[pos_central] = self.elemento[i];
                }else if i % 2 != 0{
                    temp[pos_central - izq] = self.elemento[i];
                    izq += 1;
                }else{
                    temp[pos_central + der] = self.elemento[i];
                    der += 1;
                }
            }
            for i in 0..self.dimension {
                self.elemento[i] = temp[i];
            }
        }

    }
    fn eliminar_duplicados(&mut self){
        let mut i = 0;
        while i < self.dimension{
            let mut j = i +1;
            while j < self.dimension{
                if self.elemento[i] == self.elemento[j]{
                    let mut k = j;
                    while k < self.dimension - 1{
                        self.elemento[k] = self.elemento[k+1];
                        k+=1;
                    }
                    self.dimension -=1;
                }else{
                    j+=1;
                }
            }
            i+=1;
        }
    }

}



fn main() {
    //crear una instancia

    let mut v = Vectores ::new();
    v.addelement(9);
    v.addelement(2);
    v.addelement(1);
    v.addelement(0);
    v.addelement(7);
    v.addelement(19);
    v.addelement(8);
    v.addelement(3);


    println!("----------------------------");

    v.mostrar();
    println!("-----------------------------");
    println!("La cantidad de elementos es :  {}", v.dim());


    v.insertar(7,5);
    println!("la cantidad de elementos es : {}", v.dim());

    v.dimensionar(10);

    v.mostrar();
    println!("la cantidad es :{}" , v.dim());

    println!("la posicion del elemento es {}", v.buscar(19));

    //println!("el elemento eliminado es : {}", v.elimina(8));
    
    v.mostrar();

    v.ordenar();
     
    println!("--------------------");
    v.mostrar();
    
    v.elimina_impar();


    println!("--------------------");
    v.mostrar();

    
    v.elimina_par();

    println!("--------------------");
    v.mostrar();

    let mut v1 = Vectores ::new();
    v1.addelement(5);
    v1.addelement(4);
    v1.addelement(3);
    v1.addelement(2);
    v1.addelement(1);
  
    
    println!("--- EJERCICIO 1: IMPARES Y PARES ---");
     v1.ordenar_impares_pares();
     println!("Ordenado: ");
      v1.mostrar(); // Esperado: [ 1, 3, 5, 2, 4 ]

      println!("---------------------------------------");
    // v1.ordenar_burbuja();

     //v1.mostrar();

    let mut v2 = Vectores ::new();


     println!("---------------------------------------");
     v2.addelement(1);
    v2.addelement(6);
    v2.addelement(5);
    v2.addelement(2);
    v2.addelement(0);
    v2.addelement(4);
    v2.addelement(3);

     println!("--- EJERCICIO 2: ADENTRO 
     HACIA AFUERA (Ejemplo 1) ---");
     v2.ordenar_adentro_hacia_afuera();
     println!("Ordenado: "); 
     v2.mostrar();




}

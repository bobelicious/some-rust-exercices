use pessoas::Pessoa;
use sexo::Sexo;

pub mod pessoas;
pub mod sexo;

fn main() {
    let p1= Pessoa::new_pessoa(sexo::Sexo::M, 1.80);
    let p2= Pessoa::new_pessoa(sexo::Sexo::M, 1.75);
    let p3= Pessoa::new_pessoa(sexo::Sexo::M, 1.93);
    let p4= Pessoa::new_pessoa(sexo::Sexo::M, 1.70);
    let p5= Pessoa::new_pessoa(sexo::Sexo::M, 1.65);
    let p6= Pessoa::new_pessoa(sexo::Sexo::F, 1.50);
    let p7= Pessoa::new_pessoa(sexo::Sexo::F, 1.55);
    let p8= Pessoa::new_pessoa(sexo::Sexo::F, 1.60);
    let p9= Pessoa::new_pessoa(sexo::Sexo::F, 1.63);
    let p10= Pessoa::new_pessoa(sexo::Sexo::F, 1.45);

    let alunos: Vec<pessoas::Pessoa> = vec![
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
        ];

    min_max(&alunos);

    num_mulher(&alunos);

    media(&alunos)
}

fn min_max (alns: &Vec<Pessoa>) {
    let mut  max: f64 = 0.0;
    let mut min: f64 = 0.0;

    for a in alns{

        if a.altura > max {
            max = a.altura
        }
        if a.altura < min || min == 0.0 {
            min = a.altura
        }
    };
    println! ("Maior aluno tem altura de {:?} e o menor aluno tem altura de {:?}", max, min);
}

fn media (alunos: &Vec<Pessoa>){
    let mut num_aluno: f64 = 0.0;
    let mut altura: f64 = 0.0;

    for aluno in alunos{
        match aluno.gender {
            Sexo::M =>{
                num_aluno +=1.0;
                altura += aluno.altura;

            },
            _=> ()
        };
    }

    let media: f64 = altura / num_aluno;

    println!("A media de altura é {:?}", media)
}

fn num_mulher (alunos: &Vec<Pessoa>){
    let mut n: i32 = 0;
    for alns in alunos {
        match alns.gender {
            Sexo::F=>{
                n +=1;
            }
            _ => ()
        }
    }
    println!("O numero de alunas são {:?}", n)
}
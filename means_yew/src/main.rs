use std::vec::Vec;

fn main() 
{
	let mut student = Student::student(String::from("Gonçalo Salgueirinho"), String::from("Engenharia Informática"));
	let subject = Subject::subject(String::from("Análise Matemática"), 17, 6);
	let subject2 = Subject::subject(String::from("Algebra"), 12, 5);
	student.add_subject(subject);
	student.add_subject(subject2);
	student.mean();
	println!("{}", student.display());	

	return;
}
#[derive(Debug)]
struct Student
{
	name: String,
	graduation: String,
	mean: f32,
	subjects: Vec<Subject>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Subject
{
	name: String,
	grade: u32,
	ects: u32,
}

impl Subject
{
	fn subject(name: String, grade: u32, ects: u32) -> Subject
	{
		Subject
		{
			name,
			grade,
			ects,
		}
	}
	/*
	fn name(&self) -> String 
	{
		self.name.clone()
	}
	*/
	fn grade(&self) -> u32
	{
		self.grade
	}
	fn ects(&self) -> u32
	{
		self.ects
	}
	
}
	
impl Student
{
	fn student(name: String, graduation: String) -> Student
	{
		Student
		{
			name,
			graduation,
			mean: 0.0,
			subjects: Vec::new(),
		}
	}
	fn add_subject(&mut self, sub: Subject)
	{
		self.subjects.push(sub);
	}
	fn calculate_mean(&mut self)
	{ 
		let mut grades: u32 = 0;
		let mut ectss: u32 = 0;
		for sub in &self.subjects
		{
			grades += sub.grade() * sub.ects();
			ectss += sub.ects();
		}
		let grades = grades as f32;
		let ectss = ectss as f32;
		self.mean = grades / ectss;
	}
	fn display(&self) ->	String
	{
		let mut str = String::new();
		str = format!("{}Name: {}\nGraduation: {}\nMean: {:.2}",
            str,self.name, self.graduation, self.mean);
		return str;
	}
	
	fn mean(&mut self) -> f32
	{
		self.calculate_mean();
        self.mean
	}
    /*
	fn name(&self) -> String
	{
		self.name.clone()
	}
	fn graduation(&self) -> String
	{
		self.graduation.clone()
	}
	*/
}


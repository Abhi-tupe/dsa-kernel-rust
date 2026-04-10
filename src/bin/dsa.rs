fn main(){
    fn abhi(){
        //let message = "hello abhi this side";
        for mut i in 1..=12{
            println!("2*{}={}",i,2*i);
            i += 1;
            if i==11{
                break;
            }
            
        }
    }
    abhi();
    //if you call println!("{}",message);
    //it will show error of not found in this scope look for {}

    fn add(a:i32,b:i32)->i32{
        return a+b;
    }
    let lap = add(7,8);
    println!("using return and pointer{}",lap);

    let _a = 12;
    let a = 14;
    println!("example of shadowing {}",a);
    //in this case the value of a is 14 because of shadowing as it shadows the previous value of a which is 12
    let y = 12;
    {
        let y = 14;
        println!("example of shadowinf and scoping but inside the block {}",y);
    }
    println!("this will print out of the block value which is {}",y);

    let mut ab = "This is my most latest practise of ".to_string();
    ab.push_str("rust");
    println!("{}",ab);
    ab.push('!');
    println!("{}",ab);

    let str1 = "hello".to_string();
    let str2 ="world".to_string();
    let str3 = "nice".to_string();
    let str4 = format!("{} {} {}",str1,str2,str3);
    println!("{}",str4);
    let str5 = str1 + " " + &str2 + " " + &str3;
    println!("{}",str5);
    println!("{}",str5.len()); 

    let bc = "hello world".to_string();
    let cb = bc;
    println!("{}",cb);
    //if we try to print bc it will show error because of ownership transfer to cb and bc is no longer valid also case of ownership transfer is not applicable for primitive data types like integers and floats because they implement the Copy trait which allows them to be copied rather than moved when assigned to another variable or passed to a function.

    let cd = 12;
    let dc = cd;
    println!("{}",dc);

    let cc ='b';
    let dd = cc;
    println!("{}",dd);
    //here it will work because for simple types like num , bools and characters the ownership is not transferred but copied because they implement the Copy trait which allows them to be copied rather than moved when assigned to another variable or passed to a function.

    let aa = "abhishek".to_string();
    let bb = aa.clone();
    println!("using clone method {}",bb);
    //here we cloned it so both aa and bb will print same output

    let de = "hii my name is abhishek".to_string();
    let ed = &de;
    println!("{}",de);
    println!("{}",ed);//we use & for reference in which you don't own the value just reference it or borrow it and it will not transfer ownership to ed so both de and ed will print same output

    let mut ef = "hii now the time is 23:57".to_string();
    let fe =&mut ef;
    fe.push_str("i guess i'll sleep after this");
    println!("{}",fe);

    let table = [2, 3 ,4, 5,6, 7, 8,9,10, 11];
    for mut i in table{
        println!("{}",i);
        println!("2*{}={}",i,2*i);
        i +=1;
        if i ==11{
            break;
        }
    }
    println!("this array had {} elements",table.len());
    println!("{:?}",table);
    println!("{}",table[3]);

    let mut class = vec!["abhi" , "rushi", "Raj", "Shreya"];
    class.push("shreyash");
    println!("{:?}",class);
    class.remove(1);
    class.pop();
    println!("{:?}",class);
    class.insert(1,"rushi");
    class.insert(2 , "pankaj");
    println!("{:?}",class);
    println!("{}",class.len());
    for mut j in &class{
        if *j == "shreya"{
            println!("shreya has been found");
        }
        else{
            println!(" those are the valuse before finding shreya : -{}",j);
        }
    }  // in this why the output shreya has been found didn't pop up because we can't increment string like numbers we can't use  i += 1 here because it will not follow the rules here 
    //and *j is dereference as &class shows reference both nullify each other 
    //we can write the code like this too
    // for j in class {
    // if j == "shreya" {
    //     println!("shreya has been found");
    // }}
    let detail = ("abhishek","Tupe", 20 , true);
    println!("the full name of the candidate is {} {} ",detail.0,detail.1);
    println!("the age of the candidate is {}",detail.2);
    if detail.2 > 18{
        println!("the candidate is eligible for exam {}",detail.3);
    }
    let (name , surname , age ,eligible) = detail;
    println!("the full name of the candidate is {} {} ",name,surname);
    println!("the age of the candidate is {}",age);
    if age > 18{
        println!("the candidate is eligible for exam {}",eligible);
    }

    fn vote() -> (String, i32){
         ("pushpa".to_string(),33)

    }
    let char = vote();
    println!("the name of the person is {} and his age 2 years ago was {}", char.0,char.1);






}

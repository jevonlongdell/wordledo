#![allow(non_snake_case)]


extern crate rayon;

//use lazy_static::lazy_static;
//use regex::Regex;
//use std::fs::File;
//use std::path::Path;
//use std::io::BufReader;
use std::io::stdin;
//use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use libm::exp;
use libm::log2;
use std::cmp::min;
use itertools::izip;
use ::permutation::*;
use indexmap::IndexMap;
use rayon::prelude::*;

//use crate::wordlist::wordlist;

mod wordlist;


//used to describe the info held about the mystery word
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Clues {
    green: Vec<char>,
    orange: Vec<Vec<char>>,
    grey: Vec<char>,
}
        


// found this after leaving the code for a while
// not sure what I was thinking
// doesn't compile

//fn make_clues_to_words(wordslist :Vec<String>) -> HashMap<Clues,Vec<String>> {
//    let mut clues_to_words = HashMap<Clues,Vec<String   >> = HashMap::new();
//    
//
//}



fn cal_resulting_entropy(guess: &str, possiblewords: &Vec<&str>, words: &IndexMap<String,f64>, pnorm: f64, clues: &Clues) -> f64 {
    // Calculates the resulting entropy for a particular guess
    // given the possible mystery words that are left.
  

    
    let mut outcomes: HashMap<Clues,Vec<&str>> = HashMap::new();

    // # for a given guess goes through all the possible mystery
    // # words and works out what the outcome (Clues) would be for each
    // # of those possible mystery words
    // # returns a dict of which takes the clue info
    // # and maps to a list of possible mystery words
    
    for mysteryword in possiblewords.iter(){

        //make empt set of clues 
        let mut green: Vec<char> = clues.green.clone();
        let mut orange: Vec<Vec<char>> = clues.orange.clone();
        let mut grey =  clues.grey.clone();
        
        
        //fill out the greeen orange and grey for the guess and this mystery word
        for (wchar,greenchar,orange_item,guesschar) in izip!(mysteryword.chars(),green.iter_mut(),orange.iter_mut(),guess.chars()){
            if wchar == guesschar {
                *greenchar = guesschar ;
            } 
            else if mysteryword.contains(guesschar) {
                orange_item.push(guesschar);
            }
            grey.push(guesschar);
        }
        
        
        let info = Clues{
            green: green.clone(),
            orange: orange.clone(),
            grey: grey.clone(),
        };
        
        //if info is in our list 
        if outcomes.contains_key(&info){
            outcomes.get_mut(&info).unwrap().push(mysteryword);
        } else {
            outcomes.insert(info, vec![mysteryword]);
        }
        // If we've seen theese clues before then add them to list of words
        // otherwise add a new key to hash table.

        //would be nice to do something like this if I could work out how
        // match outcomes.get_mut(&info){
        //     Some(x) => x.push(*mysteryword),
        //     None => outcomes.insert(info, vec![mysteryword]),
        // }
        }

    // Work out the entropy for out guess

    let mut H = 0.0;
    
    for (_info,wds) in outcomes.iter(){
        let mut poutcome = 0.0; //probablility of this outcome
        let mut Houtcome = 0.0; //entropy if this is the outcome
        for w in wds {
            let p = words.get(*w).unwrap()/pnorm;
            poutcome += p;
        }
        for w in wds {
            let p = words.get(*w).unwrap()/(pnorm*poutcome);
            Houtcome += - p * log2(p);
        }
        H+= poutcome*Houtcome;
    }
    return H;            
    
}

    
    
    fn main() {
        
        
        //The "words" hash map stores the prior
        //probability for each word. 
        //we take the prior proability to be 
        //prob = exp(-k/3000) where k is the index of the word
        //in a list sorted by popularity
        //we read this list in from wordlist.txt 
        
        //let wordlistpath = Path::new("wordlist.txt");
        //let f = File::open(&wordlistpath).unwrap();
        //let reader =  BufReader::new(f);
        
        let mut words: IndexMap<String,f64> = IndexMap::new();
        let mut probsum =0.0;
        
        //for (k, line) in reader.lines().enumerate() {
        //    let p= exp(-(k as f64) / 3000.0);
        //    probsum += p;
        //    words.insert(line.unwrap(), p);
        //}
        for (k,w) in wordlist::allwords().iter().enumerate(){
            let p= exp(-(k as f64) / 3000.0);
            probsum += p;
            //println!("{:?}",w);
            words.insert(w.to_string(),p);
        }

        for (_, p) in words.iter_mut(){
            *p /= probsum;
        }
  
        
        
        // read in the green, orange and grey letters from stdin
        
        println!("Enter green letters. The ones you know for sure, put a dot for the ones you don't know (in form h.l..)");
        let mut greenstring =  String::new();
        stdin().read_line(&mut greenstring).unwrap();
        let mut greenstring = greenstring.trim();
        
        if greenstring=="" {
            greenstring= ".....";
        }
        assert_eq!(greenstring.len(),5);
        
        println!();
        println!();
        
        println!("Enter orange letters for each of the five letters, separated by four spaces, dots ignored (something like \"gs . s . .\"");
        let mut orangestring = String::new();
        stdin().read_line(&mut orangestring).unwrap();
        let mut orangestring = orangestring.trim();
        if orangestring==""{
            orangestring=". . . . .";
        }
        
        
        println!();
        println!();
        println!("Enter letters that you have tried so far (with no spaces between them, any order)\n just hit enter if there are none");
        let mut greystring = String::new();
        stdin().read_line(&mut greystring).unwrap();
        let greystring = greystring.trim();
        
        // turning user input in to datatype useful for 
        // subsequent computation
        let priorgreen = greenstring;
        
        let mut priororange: Vec<&str> = orangestring.split_whitespace().collect();
        for s in priororange.iter_mut(){
            if *s=="."{
                *s = "";
            }
        }
        assert_eq!(priororange.len(),5);
        
        let mut priorgrey = HashSet::new();
        for c in greystring.chars() {
            if !c.is_whitespace(){
                priorgrey.insert(c);
            }
        }
        // remove the green and orange letters from the grey ones
        for c in priorgreen.chars(){
            priorgrey.remove(&c);
        }
        for s in &priororange{
            for c in s.chars(){
                priorgrey.remove(&c);
            }
        }
        
        // make a list of possible solutions
        let mut possiblewords:Vec<&str> = Vec::new();
        
        'possiblewordsearch: 
        for (w,_) in words.iter(){
            
            for (wchar,gchar) in w.chars().zip( priorgreen.chars() ){
                //if it doesn't match green give up on this word
                if gchar!='.' &&  gchar!=wchar {
                    continue 'possiblewordsearch;
                }
                // if it has a grey letter give up
                if priorgrey.contains(&wchar){
                    continue 'possiblewordsearch;
                }
                
            }   
            
            let wordletters: HashSet<char> = w.chars().collect();
            for (o,wchar) in priororange.iter().zip(w.chars()){
                let wcharset = HashSet::from([wchar]);
                let wordletters_not_this_slot: HashSet<_> = wordletters.difference(&wcharset).collect();
                for ochar in o.chars(){
                    if !wordletters_not_this_slot.contains(&ochar){
                        continue 'possiblewordsearch
                    }
                    //if wchar==ochar {
                    //    continue 'possiblewordsearch
                    //}   
                }
                //orange leters are not green
                
            }
            
            possiblewords.push(w)
            
        }
        
        //Calculate current (initial) entropy 
        let n = possiblewords.len();
        
        if n==0{
            println!("No compatibile words found");
            return;
        }
        
        if n==1{
            println!("The answer is {}!",possiblewords[0]);
            return;
        }
        
        let nw= min(30,n);
        println!("Found {} possilbe words, the first {} are:",n,nw);
        for w in possiblewords.iter().take(nw){
            print!("{} ",w)
        }
        println!(); println!();
        
        
        let mut Hinit = 0.0;
        let mut probsum=0.0;
        for w in possiblewords.iter(){
            let p = words.get(w as &str).unwrap();
            probsum+=p;
            //println!("{} {}",probsum,p);
        }
        for w in possiblewords.iter(){
            let p = words.get(w as &str).unwrap();
            Hinit -= p/probsum *log2(p/probsum);
        }
        
        
        println!("Initial entropy = {:.1} bits",Hinit);
        

        

        let mut Hvals :IndexMap<String,f64> = IndexMap::new();
       
        // Blank set of clues for cal_resulting_entropy
        // should populate these to make the entropy calculated more correct,
        // I think.
        let clues = Clues {
            green: ".....".chars().collect(),
            orange:  vec![Vec::new(), Vec::new(),Vec::new(),Vec::new(),Vec::new()],
            grey:   Vec::new(),
        };
        
        let wordvec: Vec<_> = words.keys().collect();
        let Hvec :Vec<f64> = wordvec.par_iter().map(|guess| cal_resulting_entropy(guess, &possiblewords, &words, probsum,&clues)).collect();
        
        let mut hbest = Hinit;

        for (guess, entropy) in wordvec.iter().zip(Hvec.iter()) {
            if *entropy < hbest {
                hbest=*entropy;
            }            
            Hvals.insert(guess.to_string(),*entropy);
        }

        
        println!();println!();
        println!("Possible solutions:");
        println!("{:?}",possiblewords);


        println!();println!();
        println!("Possible solutions and resulting entropy");

        possiblewords.sort_by(|a,b| Hvals.get(a as &str).unwrap().partial_cmp(Hvals.get(b as &str).unwrap()).unwrap());
        for w in possiblewords.iter().take(40){
            print!("({}, {:.2}), ", w, Hvals.get(w as &str).unwrap());
        }
        



        println!();println!();
        println!("Words that will narrow down what the answer are");
        let ordering = permutation::sort_by(&Hvec,|a, b| a.partial_cmp(b).unwrap());
        let Hvec = ordering.apply_slice(Hvec);
        let wordvec = ordering.apply_slice(wordvec);
        for (k,(w,h)) in wordvec.iter().zip(Hvec.iter()).enumerate(){
            print!("({},{:.2}), ",w,h);
            if k > 40 {
                break;
            }
        }
        println!();println!();
        
        
    }

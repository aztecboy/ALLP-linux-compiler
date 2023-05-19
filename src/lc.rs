




//a struct to store strings

pub struct StringTable{
    string_data:Vec<char>,
    string_lengths:Vec<u64>,
    string_labels:Vec<String>

}
// a struct to store data in rodata 
pub struct RoDataTable{
    data:Vec<u8>,
    data_lengths:Vec<u64>,
    data_labels:Vec<String>

    

}

//a struct to store data in data
pub struct DataTable{
    data:Vec<u64>,
    data_lengths:Vec<usize>,
    data_labels:Vec<String>,
    data_types:Vec<u8>,

}

//a struct to store all of the file data
pub struct FileData{
    pub string_table:StringTable,
    pub rodata_table:RoDataTable,
    pub data_table:DataTable,
    pub flat_code:String,
    pub file_name:String,
    pub lld_path:String,
    pub fasm_path:String
    
}
//A module to contain addresses
pub struct Address{
    label:String,
    used:bool,

}
// a module to contain a register
pub struct Register<'a>{
    register:&'a str

}
//a module containing all the file paths
mod paths{
    pub const PATH_TO_FASM:&str="./fasm";
    pub const PATH_TO_LLD:&str="/bin/ld.lld";

}
// a module for containing all the errors for the library
mod errors{
    
    pub mod instructions{
        pub mod registers{
            pub const INVALID_REGISTER:&str="invalid register";

        }
        pub mod mov{
            pub mod reg_to_reg{
                

            }

        }
        pub mod create_data{
            pub mod check_label{
                pub const INVALID_LABEL:&str="invalid label";
            }
            pub mod check_address{
                pub const INVALID_ADDRESS:&str="invalid address";
            }

        }

    }
    pub mod check_for_depends{
        pub const FASM_NOT_FOUND:&str="unable to find fasm";
        pub const LLD_NOT_FOUND:&str="unable to find lld";

    }
    pub mod spawn_func{
        pub const INVALID_PARAMETERS:&str="invalid parameters for the spawn function";
    }
}
//a wrapper for fasm
mod flat_wrapper64{
    pub const data_section:&str="format ELF64\nsection \".data\" writeable\n";
    pub const text_section:&str="format ELF64\nsection \".text\" executable\n";
    //a function to compile fasm
    pub fn compile_flat(file:String,path:&String)->Result<(),&'static str>{
        
        let fasm_output:String=match std::process::Command::new(path).arg(file).status(){
            Ok(out)=>out.to_string(),
            Err(error)=>error.to_string()
    
        };
        
        return Ok(());
    }
    

}
// a wrapper for lld
mod lld_wrapper64{
     //a function to link all of the object files together
    pub fn link(text:String,data:String,output:&String,path:&String)->Result<(),&'static str>{

        let lld_output:String=match std::process::Command::new(path).args(&["-r".to_string(),"-o".to_string(),format!("{}.o",output.to_string()),text,data]).status(){
            Ok(out)=>out.to_string(),
            Err(error)=>error.to_string()
    
        };
        
        return Ok(());

    }
    pub fn make_execution(file:&String,output:&String,path:&String)->Result<(),&'static str>{
        let lld_output:String=match std::process::Command::new(path).args(&["-o",output,&format!("{}.o",file)]).status(){
            Ok(out)=>out.to_string(),
            Err(error)=>error.to_string()
    
        };
        
        return Ok(());
    }
    
}
// the types of variables
pub mod data_types{
    pub const NULL_TYPE:u8=0x00;
    pub const INTEGER_8:u8=0x01;
    pub const INTEGER_16:u8=0x02;
    pub const INTEGER_32:u8=0x03;
    pub const INTEGER_64:u8=0x04;
    pub const STRING_NT:u8=0x05;
    pub const STRING_ST:u8=0x06;
    pub const BYTE_ARRAY_8:u8=0x07;

}

//a list of all x86_64 registers
pub mod registers_x86 {
    use super::*;
    
    pub const RAX: Register = Register { register: "rax" };
    pub const RBX: Register = Register { register: "rbx" };
    pub const RCX: Register = Register { register: "rcx" };
    pub const RDX: Register = Register { register: "rdx" };
    pub const RSI: Register = Register { register: "rsi" };
    pub const RDI: Register = Register { register: "rdi" };
    pub const RBP: Register = Register { register: "rbp" };
    pub const RSP: Register = Register { register: "rsp" };
    pub const R8: Register = Register { register: "r8" };
    pub const R9: Register = Register { register: "r9" };
    pub const R10: Register = Register { register: "r10" };
    pub const R11: Register = Register { register: "r11" };
    pub const R12: Register = Register { register: "r12" };
    pub const R13: Register = Register { register: "r13" };
    pub const R14: Register = Register { register: "r14" };
    pub const R15: Register = Register { register: "r15" };
    pub const EAX: Register = Register { register: "eax" };
    pub const EBX: Register = Register { register: "ebx" };
    pub const ECX: Register = Register { register: "ecx" };
    pub const EDX: Register = Register { register: "edx" };
    pub const ESI: Register = Register { register: "esi" };
    pub const EDI: Register = Register { register: "edi" };
    pub const EBP: Register = Register { register: "ebp" };
    pub const ESP: Register = Register { register: "esp" };
    pub const R8D: Register = Register { register: "r8d" };
    pub const R9D: Register = Register { register: "r9d" };
    pub const R10D: Register = Register { register: "r10d" };
    pub const R11D: Register = Register { register: "r11d" };
    pub const R12D: Register = Register { register: "r12d" };
    pub const R13D: Register = Register { register: "r13d" };
    pub const R14D: Register = Register { register: "r14d" };
    pub const R15D: Register = Register { register: "r15d" };
    pub const AX: Register = Register { register: "ax" };
    pub const BX: Register = Register { register: "bx" };
    pub const CX: Register = Register { register: "cx" };
    pub const DX: Register = Register { register: "dx" };
    pub const SI: Register = Register { register: "si" };
    pub const DI: Register = Register { register: "di" };
    pub const BP: Register = Register { register: "bp" };
    pub const SP: Register = Register { register: "sp" };
    pub const R8W: Register = Register { register: "r8w" };
    pub const R9W: Register = Register { register: "r9w" };
    pub const R10W: Register = Register { register: "r10w" };
    pub const R11W: Register = Register { register: "r11w" };
    pub const R12W: Register = Register { register: "r12w" };
    pub const R13W: Register = Register { register: "r13w" };
    pub const R14W: Register = Register { register: "r14w" };
    pub const R15W: Register = Register { register: "r15w" };
    pub const AL: Register = Register { register: "al" };
    pub const BL: Register = Register { register: "bl" };
    pub const CL: Register = Register { register: "cl" };
    pub const DL: Register = Register { register: "dl" };
    pub const SIL: Register = Register { register: "sil" };
    pub const DIL: Register = Register { register: "dil" };
    pub const BPL: Register = Register { register: "bpl" };
    pub const SPL: Register = Register { register: "spl" };
    pub const R8L: Register = Register { register: "r8l" };
    pub const R9L: Register = Register { register: "r9l" };
    pub const R10L: Register = Register { register: "r10l" };
    pub const R11L: Register = Register { register: "r11l" };
    pub const R12L: Register = Register { register: "r12l" };
    pub const R13L: Register = Register { register: "r13l" };
    pub const R14L: Register = Register { register: "r14l" };
    pub const R15L: Register = Register { register: "r15l" };
}
//a function to turn a u8 vector value to a 64 vector value
fn u8_to_u64_vector(u8_vector:Vec<u8>)->Vec<u64>{
    return u8_vector.iter().map(|&x| x as u64).collect()
        
}
// a module containing machine instructions
pub mod instructions_x86{
    use super::*;
    //makes the _start label
    pub fn create_entry(mut file_data:FileData)->Result<FileData,&'static str>{
        file_data.flat_code+=&String::from("public _start\n_start:\n");
        return Ok(file_data);
    }
    // makes syscall
    pub fn system_call(mut file_data:FileData)->Result<FileData,&'static str>{
        file_data.flat_code+="syscall\n";
        return Ok(file_data);
    }
    
  
    

    pub mod math{
        use super::super::*;
        // a function that makes an increment string
        fn increment_value_8(string_to_increment:String)->String{
            return format!("inc byte [{}]\n",string_to_increment);
        
        }
        fn increment(string_to_increment:String)->String{
            return format!("inc {}\n",string_to_increment);
        
        }
        // increments an address value
        pub fn increment_address_value_8(address:Address, mut file_data:FileData)->Result<FileData,&'static str>{
            file_data.flat_code+=&increment_value_8(instructions_x86::create_data::get_label_from_address(&address).to_string());
            return Ok(file_data);
            
    

        }
        // increments a register
        pub fn increment_register(register:&Register, mut file_data:FileData)->Result<FileData,&'static str>{
            file_data.flat_code+=&increment(instructions_x86::registers::get_label_from_register(&register).to_string());
            return Ok(file_data);
            
    

        }
        

    }
    // a module containing all of the instructions for creating data
    pub mod create_data{
        use super::super::*;
        //a list of invalid lables
        pub const INVALID_LABELS:[&str;1]=["test"];

        pub fn place_address(address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.flat_code+=&(get_label_from_address(&address).to_owned()+":\n");
            return Ok(file_data);

        }
        //creates an address
        pub fn create_address(label:String)->Address{
            check_label(&label);
            let address=Address{
                label:label,
                used:false
            };
            return address;
        }
        //adds extrn to a string
        fn extern_string(string_to_extern:String)->String{
            return String::from("extrn ")+&string_to_extern+&String::from("\n");
        }

        //externs a label
        pub fn extern_label(label:&String,mut file_data:FileData)->Result<FileData,&'static str>{
            file_data.flat_code+=&extern_string(label.to_string());
            return Ok(file_data);

        }
        //checks the label
        pub fn check_label(label:&String)->Result<(),&'static str>{
    
            let mut error:bool=false;

            for i in 0..INVALID_LABELS.len(){
                if &label==&(&INVALID_LABELS[i].to_string()){
                    
                    error=true;
                    break;
                }
            }
            if error{
                return Err(errors::instructions::create_data::check_label::INVALID_LABEL);
            }
            return Ok(());

        }
        //checks an address
        pub fn check_address(address:&Address)->Result<(),&'static str>{
            if address.used==true{
                return Err(errors::instructions::create_data::check_address::INVALID_ADDRESS);
            }
            return Ok(());
        }
        //sets the used bool in address as true
        pub fn set_address_used(address:&mut Address){
            address.used=true;
        }
        //gets the string in label in the address
        pub fn get_label_from_address(address:&Address)->&String{
            return &address.label;
        
        }

        // puts the length into data table
        fn create_length(length:usize,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.data_table.data_lengths.push(length);

            return Ok(file_data);
        }
        // puts the label into data table
        fn create_label(label:&String,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.data_table.data_labels.push(label.to_string());
            return Ok(file_data);

        }
        // puts a type into the data table
        fn create_type(type1:u8,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.data_table.data_types.push(type1);

            return Ok(file_data);

        }
        // puts a byte into the data table
        fn create_byte(byte:u8,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.data_table.data.push(byte.into());

            return Ok(file_data);

        }
        // puts a 32 bit byte into the data table
        fn create_byte_32(byte:u32,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.data_table.data.push(byte.into());

            return Ok(file_data);

        }
        // puts a 64 bit byte into the data table
        fn create_byte_64(byte:u64,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.data_table.data.push(byte);

            return Ok(file_data);

        }
        // makes a 8 bit array
        fn create_array_8(mut array:Vec<u8>,mut file_data:FileData)->Result<FileData,&'static str>{
            let mut u64_array:Vec<u64>=u8_to_u64_vector(array);
            file_data.data_table.data.append(&mut u64_array);
            return Ok(file_data);
        }
       
        //makes a integer value
        pub fn create_integer_8(int1:u8,address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
            check_address(&address)?;
            file_data=create_label(get_label_from_address(&address),file_data).unwrap();

            file_data=create_type(data_types::INTEGER_8,file_data).unwrap();

            file_data=create_length(1,file_data).unwrap();

            file_data=create_byte(int1,file_data).unwrap();

            file_data=extern_label(get_label_from_address(&address),file_data).unwrap();
            
            

            return Ok(file_data);

        }
        //makes a 32 bit integer value
        pub fn create_integer_32(int1:u32,address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
            check_address(&address)?;
            file_data=create_label(get_label_from_address(&address),file_data).unwrap();

            file_data=create_type(data_types::INTEGER_32,file_data).unwrap();

            file_data=create_length(1,file_data).unwrap();

            file_data=create_byte_32(int1,file_data).unwrap();

            file_data=extern_label(&get_label_from_address(&address),file_data).unwrap();

            return Ok(file_data);

        }
        //makes a 64 bit integer value
        pub fn create_integer_64(int1:u64,address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
            check_address(&address)?;
            file_data=create_label(get_label_from_address(&address),file_data).unwrap();

            file_data=create_type(data_types::INTEGER_64,file_data).unwrap();

            file_data=create_length(1,file_data).unwrap();

            file_data=create_byte_64(int1,file_data).unwrap();

            file_data=extern_label(get_label_from_address(&address),file_data).unwrap();

            return Ok(file_data);

        }
        // makes a byte array
        pub fn create_byte_array_8(array:Vec<u8>,address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
            check_address(&address)?;
            file_data=create_label(get_label_from_address(&address),file_data).unwrap();

            file_data=create_type(data_types::BYTE_ARRAY_8,file_data).unwrap();

            file_data=create_length(array.len(),file_data).unwrap();

            file_data=create_array_8(array,file_data).unwrap();

            file_data=extern_label(get_label_from_address(&address),file_data).unwrap();
            
            return Ok(file_data);
            
        }


    }

    // a module containing all of the instructions for registers
    pub mod registers{
        
        use super::super::*;
        // a list of register
        pub const VALID_REGISTERS: [&str; 57] = [
        "rax", "rcx", "rbx", "rcx", "rdx", "rbp", "rsp", "rsi", "rdi",
        "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
        "eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp",
        "ax", "bx", "cx", "dx", "si", "di", "bp", "sp",
        "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w",
        "al", "bl", "cl", "dl", "sil", "dil", "bpl", "spl",
        "r8l", "r9l", "r10l", "r11l", "r12l", "r13l", "r14l", "r15l",
        ]; // To add another register to the array, add a string and increment the array size
                                                    
        // a fuction to check if the register is a valid register
        pub fn check_register(register:&String)->Result<(),&'static str>{

            let mut error:bool=true;

            for i in 0..VALID_REGISTERS.len(){
                if &register==&(&VALID_REGISTERS[i].to_string()){
                    
                    error=false;
                    break;
                }
            }
            if error{
                return Err(errors::instructions::registers::INVALID_REGISTER);
            }
            return Ok(());

            
        }
        // a function to get the label from the register struct
        pub fn get_label_from_register(register:&Register)->String{
            
            return register.register.to_string();
            

        }

        

    }
    // a module containing all of the instructions to transfer data
    pub mod transfer{
        use super::super::*;
        //jumps to the address
        pub fn jump_to_address(address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
            file_data.flat_code+=&format!("jmp {}",instructions_x86::create_data::get_label_from_address(&address));
            return Ok(file_data);
        
        }
        // a function to construct a mov instruction
        fn create_mov(value:String,value2:String)->String{
            return format!("mov {},{}\n",value,value2);

        }
        // puts [] around a string so it can be indirect access
        fn indirect_access(label:String)->String{
            return format!(" [{}] ",label);

        }
        // a function to move a register value to a register 

        pub fn register_to_register(register1:&Register,register2:&Register,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.flat_code+=&create_mov(instructions_x86::registers::get_label_from_register(&register1),instructions_x86::registers::get_label_from_register(register2));
            return Ok(file_data);
            
        }
        //moves a byte value to register
        pub fn byte_to_register(int1:u8,register:&Register,mut file_data:FileData)->Result<FileData,&'static str>{
            
            file_data.flat_code+=&create_mov(instructions_x86::registers::get_label_from_register(&register),format!("0x{:02x}",int1));
            return Ok(file_data);

        }
        // move an address value to register
        pub fn address_value_to_register(address:&Address,register:&Register,mut file_data:FileData)->Result<FileData,&'static str>{
            file_data.flat_code+=&create_mov(instructions_x86::registers::get_label_from_register(&register),indirect_access(instructions_x86::create_data::get_label_from_address(&address).to_string()));
            return Ok(file_data)
        }
        // moves an register to address
        pub fn register_to_address(register:Register,address:&Address,mut file_data:FileData)->Result<FileData,&'static str>{
                    
            file_data.flat_code+=&create_mov(instructions_x86::create_data::get_label_from_address(&address).to_string(),instructions_x86::registers::get_label_from_register(&register));
            return Ok(file_data);
        }
        // moves the data inside a register to the address
        pub fn address_to_register(address:&Address,register:&Register,mut file_data:FileData)->Result<FileData,&'static str>{
            
            
            file_data.flat_code+=&create_mov(instructions_x86::registers::get_label_from_register(&register),instructions_x86::create_data::get_label_from_address(&address).to_string());
            return Ok(file_data);

        }
        
            
    }
       
}

// checks if the dependencies exist
fn check_for_depends(path_to_fasm:String,path_to_lld:String)->Result<(),&'static str>{

    if !std::fs::metadata(path_to_fasm).is_ok(){
        return Err(errors::check_for_depends::FASM_NOT_FOUND);
    }
    if !std::fs::metadata(path_to_lld).is_ok(){
        return Err(errors::check_for_depends::LLD_NOT_FOUND);
    }

    return Ok(());
}
// a function to create the file
pub fn spawn(file_name:String,mut path_to_fasm:String,mut path_to_lld:String)->Result<FileData,&'static str>{
    
    if path_to_fasm==""{

        path_to_fasm=paths::PATH_TO_FASM.to_string()

    }
    if path_to_lld==""{
    
        path_to_lld=paths::PATH_TO_LLD.to_string()
    }

    check_for_depends(path_to_fasm.clone(),path_to_lld.clone())?;
    

    let flat_code:String=flat_wrapper64::text_section.to_string();
    
    let mut file_data=FileData{
        string_table:StringTable{
            string_data:Vec::new(),
            string_lengths:Vec::new(),
            string_labels:Vec::new(),
        },
        rodata_table:RoDataTable{
            data:Vec::new(),
            data_lengths:Vec::new(),
            data_labels:Vec::new(),

        },
        data_table:DataTable{
            data:vec![0x00],
            data_lengths:vec![0x00000000],
            data_labels:vec![String::from("")],
            data_types:vec![0x00],

        },
        flat_code:flat_code,
        file_name:file_name,
        lld_path:path_to_lld,
        fasm_path:path_to_fasm
        

    };
    

    


    return Ok(file_data);

}
// a macro to panic the kernel when a internal error happens
macro_rules! internal_error{
    ()=>{panic!("an internal error has been spotted")}

}
//a module for compiling the program
pub mod compiler64{
    use super::*;
    // sets the label as public
    fn public_label(label:&String)->String{
        return format!("public {}\n",label);
    }
    // makes the null 8
    fn create_null_8(mut data_code:String)->Result<String,&'static str>{
        data_code+=&public_label(&String::from("NULL8"));
        data_code+=&String::from("NULL8:\ndb 0x00\n");
        return Ok(data_code);

    }
    //creates a integer in data
    fn create_integer_8(mut data_code:String,offset:usize,data:&Vec<u64>,label:String)->Result<String,&'static str>{
        data_code+=&public_label(&label);
        data_code+=&format!("{}:\ndb {}\n",label,data[offset+1]);
        return Ok(data_code);
    }
    //creates a integer 32 in data
    fn create_integer_32(mut data_code:String,offset:usize,data:&Vec<u64>,label:String)->Result<String,&'static str>{
        data_code+=&public_label(&label);
        data_code+=&format!("{}:\ndd {}\n",label,data[offset+1]);
        return Ok(data_code);
    }
    //creates a integer 64 in data
    fn create_integer_64(mut data_code:String,offset:usize,data:&Vec<u64>,label:String)->Result<String,&'static str>{
        data_code+=&public_label(&label);
        data_code+=&format!("{}:\ndq {}\n",label,data[offset+1]);
        return Ok(data_code);
    }
    //makes the 8 bit byte array
    fn create_byte_array_8(mut data_code:String,offset:usize,length:usize,data:&Vec<u64>,label:String)->Result<String,&'static str>{
        let mut string_to_add:String=String::new();
        for i in 1..length+1{
            if i==length{
                string_to_add+=&data[offset+i].to_string();
            }
            else{
                string_to_add+=&(data[offset+i].to_string()+",");
            }
        }
        data_code+=&public_label(&label);
        data_code+=&format!("{}:\ndb {}\n",label,string_to_add);
        return Ok(data_code);
    }
    
    //compiles the program
    pub fn compile(file_data:FileData)->Result<String,&'static str>{
        use std::io::Write;
        let mut text_file=match std::fs::File::create("text.asm"){
            Ok(file)=>file,
            Err(err)=>std::fs::File::open("./text.asm").unwrap()
        };
        let mut data_file=match std::fs::File::create("data.asm"){
            Ok(file)=>file,
            Err(err)=>std::fs::File::open("./data.asm").unwrap()
              
        };
        
        let mut data_code:String=flat_wrapper64::data_section.to_string();
        let mut typ:u8;
        let mut length:usize=0;
        let mut label:String=String::new();
        let mut data_offset:usize=0;
        let mut iterator:usize=0;
        for i in 1..file_data.data_table.data_types.len()+1{
            iterator=i-1;
            
            typ=file_data.data_table.data_types[iterator];
            length=file_data.data_table.data_lengths[iterator];
            label=file_data.data_table.data_labels[iterator].clone();

            match typ{
                data_types::NULL_TYPE=>data_code=create_null_8(data_code)?,
                data_types::INTEGER_8=>data_code=create_integer_8(data_code,data_offset,&file_data.data_table.data,label)?,
                data_types::INTEGER_32=>data_code=create_integer_32(data_code,data_offset,&file_data.data_table.data,label)?,
                data_types::INTEGER_64=>data_code=create_integer_64(data_code,data_offset,&file_data.data_table.data,label)?,
                data_types::BYTE_ARRAY_8=>data_code=create_byte_array_8(data_code,data_offset,length,&file_data.data_table.data,label)?,
                2_u8..=u8::MAX=>internal_error!()

        
            }
            data_offset+=length;
        }
        
        text_file.write_all(file_data.flat_code.as_bytes());
        data_file.write_all(data_code.as_bytes());
        flat_wrapper64::compile_flat("text.asm".to_string(),&file_data.fasm_path)?;
        flat_wrapper64::compile_flat("data.asm".to_string(),&file_data.fasm_path)?;
        lld_wrapper64::link("text.o".to_string(),"data.o".to_string(),&file_data.file_name,&file_data.lld_path);
        lld_wrapper64::make_execution(&file_data.file_name,&file_data.file_name,&file_data.lld_path);

        return Ok("compiled successful".to_string());


    }
}

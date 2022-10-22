use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use inkwell::{context::Context, module::Linkage, targets::{Target, InitializationConfig, TargetMachine}};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("source bf file to compile")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("output filename")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    
        let context=Context::create();
        let module=context.create_module("juniper ver1");
        let builder=context.create_builder();

        let i32_type=context.i32_type();
        let main_fn_type=i32_type.fn_type(&[], false);
        let main_fn=module.add_function("main", main_fn_type, Some(Linkage::External));

        let basic_block=context.append_basic_block(main_fn, "entry");
        builder.position_at_end(basic_block);

        let i32_zero=i32_type.const_int(0, false);
        builder.build_return(Some(&i32_zero));

        Target::initialize_all(&InitializationConfig::default());

        let target_triple=TargetMachine::get_default_triple();
        let cpu=TargetMachine::get_host_cpu_name().to_string();
        let features=TargetMachine::get_host_cpu_features().to_string();

        let target=Target::from_triple(&target_triple).map_err(|e| format!("{:?}",e)).unwrap();


        let target_machine=target.
        create_target_machine(&target_triple,
             &cpu, 
             &features, 
             inkwell::OptimizationLevel::Default, 
             inkwell::targets::RelocMode::Default, 
             inkwell::targets::CodeModel::Default).ok_or_else(|| "Unable to create target machine!".to_string()).unwrap();
            
             let output_filename=matches.value_of("output").unwrap();
             target_machine.write_to_file(&module, 
                inkwell::targets::FileType::Assembly, 
                output_filename.as_ref())
                .map_err(|e| format!("{:?}",e))
                .unwrap();

}

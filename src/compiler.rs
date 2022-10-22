use inkwell::{context::Context, module::{Module, Linkage}, builder::Builder, values::{PointerValue, FunctionValue}};

pub struct Compiler<'ctx>{
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

}

struct Functions<'ctx>{
    main_fn:FunctionValue<'ctx>
}

impl<'ctx> Compiler<'ctx>{

    pub fn new(context:&'ctx Context,module:Module<'ctx>,builder:Builder<'ctx>)->Compiler<'ctx>{
        Self { context, module, builder }
    }

    fn init_functions(&self)->Functions{

        let i32_type=self.context.i32_type();
        let main_fn_type=i32_type.fn_type(&[], false);
        let main_fn=self.module.add_function("main", main_fn_type, Some(Linkage::External));

        Functions { main_fn }

    }

    fn build_add_ptr(&self, amount:i32,ptr: &PointerValue){
        let i32_type=self.context.i32_type();

    }

}
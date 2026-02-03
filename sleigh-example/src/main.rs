use sleighcraft::*;

fn main() {
    let mut sleigh_builder = SleighBuilder::default();
    let spec = arch("x86").unwrap();
    let buf = [0x90, 0x32, 0x31];
    let mut loader = PlainLoadImage::from_buf(&buf, 0);
    sleigh_builder.loader(&mut loader);
    sleigh_builder.spec(spec);
    let mut asm_emit = CollectingAssemblyEmit::default();
    let mut pcode_emit = CollectingPcodeEmit::default();
    sleigh_builder.asm_emit(&mut asm_emit);
    sleigh_builder.pcode_emit(&mut pcode_emit);
    let mut sleigh = sleigh_builder.try_build().unwrap();

    sleigh.decode(0).unwrap();

    println!("{:?}", asm_emit.asms);
    println!("{:?}", pcode_emit.pcode_asms);
}

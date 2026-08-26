use super::Base;

pub struct Insert<'a> {
    args: &'a [String],
}

impl<'a> Insert<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args: args }
    }
}

impl<'a> Base for Insert<'a> {
    const OP_NAME: &'static str = "insert";

    fn validate(&self) -> std::io::Result<()> {
        let args = self.args();

        // ensure table exists

        // ensure fields are valid
        Ok(())
    }

    fn perform(&self) -> std::io::Result<()> {
        let args = self.args();
        // generate string row
        //
        // LSM -> insert into memtable (might need more like a storetable)
        //  merge into layer 1
        //  recurse down
        // B-Tree -> find appropriate page
        //  insert
        //  determine if split needed
        Ok(())
    }

    fn args(&self) -> &[String] {
        return &(self.args);
    }
}

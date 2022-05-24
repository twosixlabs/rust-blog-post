use rustplugin::myrustlib::my_foreign_interface::MyForeignInterface;


pub struct PluginWrapper {
    pub plugin: Box<dyn MyForeignInterface>,
}

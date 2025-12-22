use burn::{
    config::Config,
    module::Module,
    nn::{Linear, LinearConfig, Relu},
    tensor::{backend::Backend, Tensor},
};

#[derive(Module, Debug)]
pub struct DqnModel<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>,
    relu: Relu,
}

#[derive(Config, Debug)]
pub struct DqnModelConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,
}

impl DqnModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> DqnModel<B> {
        DqnModel {
            linear1: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            linear2: LinearConfig::new(self.hidden_size, self.hidden_size).init(device),
            linear3: LinearConfig::new(self.hidden_size, self.output_size).init(device),
            relu: Relu::new(),
        }
    }
}

impl<B: Backend> DqnModel<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(input);
        let x = self.relu.forward(x);
        let x = self.linear2.forward(x);
        let x = self.relu.forward(x);
        self.linear3.forward(x)
    }
}

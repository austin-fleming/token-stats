
pub type U256Alias = String;


pub struct TraceModel {
    pub tx_hash: String, // PK
    pub tx_position: Option<U256Alias>,
    pub block_number: U256Alias,
    pub classification: String,
    pub trace_type: String,
    pub trace_address: Vec<u8>,
    pub protocol: Option<String>,
    pub abi_name: Option<String>,
    pub function_name: Option<String>,
    pub function_signature: Option<String>,
    pub inputs: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub gas: Option<U256Alias>,
    pub value: Option<U256Alias>,
    pub gas_used: Option<U256Alias>,
    pub error: Option<String>
}


/* 
class ClassifiedTraceModel(Base):
    __tablename__ = "classified_traces"

    transaction_hash = Column(String, primary_key=True)
    transaction_position = Column(Numeric, nullable=True)
    block_number = Column(Numeric, nullable=False)
    classification = Column(String, nullable=False)
    trace_type = Column(String, nullable=False)
    trace_address = Column(ARRAY(Integer), nullable=False)
    protocol = Column(String, nullable=True)
    abi_name = Column(String, nullable=True)
    function_name = Column(String, nullable=True)
    function_signature = Column(String, nullable=True)
    inputs = Column(JSON, nullable=True)
    from_address = Column(String, nullable=True)
    to_address = Column(String, nullable=True)
    gas = Column(Numeric, nullable=True)
    value = Column(Numeric, nullable=True)
    gas_used = Column(Numeric, nullable=True)
    error = Column(String, nullable=True)
*/
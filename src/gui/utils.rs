pub fn process_input(input1: &str, input2: &str, input3: &str, topic: &str) {
    let chatter_pub = rosrust::publish(topic, 2).unwrap();
    chatter_pub.wait_for_subscribers(None).unwrap();

    // Create string message
    let msg = rosrust_msg::std_msgs::String {
        data: format!("{} {} {}", input1, input2, input3),
    };
    // Log event
    rosrust::ros_info!("Publishing: {}", msg.data);
    // Send string message to topic via publisher
    chatter_pub.send(msg).unwrap();
}
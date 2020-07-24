# OnBeat

A rust implementation of Elastic's "beat" concept - a periodic proccess for reading data and writing data to an Elasticsearch database. 
This project will eventually contain:
- A library for implemeting your own rust based beats easly - simply create a struct with your data, and it will be turned into an Elastic style document and be sent to your database
- Some implementations of existent beats such as heartbeat and filebeat - as bineries.

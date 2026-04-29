pipeline {
    agent {
        node {
            label 'docker'
        }
    }

    stages {
        stage('Stream controller') {
            stages {
                stage('Build') {
                    agent {
                        docker {
                            image 'rust:1.95.0-bookworm'
                            reuseNode true
                        }
                    }

                    steps {
                        sh 'cd stream-controller/ && cargo build --release --locked'
                    }
                }

                stage('Deploy to production') {
                    when {
                        branch 'trunk'
                    }

                    steps {
                        sshPublisher(publishers: [
                            sshPublisherDesc(
                                configName: 'uryrosesstream0',
                                transfers: [
                                    sshTransfer(
                                        sourceFiles: 'stream-controller/target/release/stream-controller',
                                        removePrefix: 'stream-controller/target/release/',
                                        keepFilePermissions: true,
                                    ),
                                ],
                                verbose: true
                            )
                        ])
                    }
                }
            }
        }

        stage('Deploy liquidsoap scripts') {
            when {
                branch 'trunk'
            }

            steps {
                sshPublisher(publishers: [
                    sshPublisherDesc(
                        configName: 'uryrosesstream0-liquidsoap',
                        transfers: [
                            sshTransfer(
                                sourceFiles: 'liq/*.liq',
                                removePrefix: 'liq/',
                                keepFilePermissions: true,
                            ),
                        ],
                        verbose: true
                    )
                ])
            }
        }
    }
}

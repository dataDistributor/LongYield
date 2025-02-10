#include <QApplication>
#include <QPushButton>
#include <QVBoxLayout>
#include <QWidget>
#include <QLabel>
#include <iostream>
#include "longyield_ffi.h"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    // Create the main window.
    QWidget window;
    window.setWindowTitle("LongYield Frontend");

    // Set up a vertical layout.
    QVBoxLayout *layout = new QVBoxLayout(&window);

    // Create a status label.
    QLabel *statusLabel = new QLabel("Status: Idle");
    layout->addWidget(statusLabel);

    // Create a "Start Mining" button.
    QPushButton *startButton = new QPushButton("Start Mining");
    layout->addWidget(startButton);

    // Create a "Get Blockchain Length" button.
    QPushButton *getLengthButton = new QPushButton("Get Blockchain Length");
    layout->addWidget(getLengthButton);

    // Connect the "Start Mining" button to the Rust function.
    QObject::connect(startButton, &QPushButton::clicked, [&]() {
        std::cout << "Starting mining..." << std::endl;
        start_mining();  // Call the Rust FFI function.
        statusLabel->setText("Status: Mining started");
    });

    // Connect the "Get Blockchain Length" button to the Rust function.
    QObject::connect(getLengthButton, &QPushButton::clicked, [&]() {
        unsigned int length = get_blockchain_length();  // Call the Rust FFI function.
        statusLabel->setText(QString("Blockchain Length: %1").arg(length));
    });

    window.setLayout(layout);
    window.resize(300, 150);
    window.show();

    return app.exec();
}

#include "mainwindow.h"
#include "ui_mainwindow.h"

#include "ijvm_cpp_wrapper.h" //NB: Not available if compile.sh not executed

#include <QFileDialog>
#include <iostream>
#include <string>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    connect(ui->actionOpen, SIGNAL(triggered()), this, SLOT(actionOpen_triggered()));
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::actionOpen_triggered(void) {
    QFileDialog dialog = QFileDialog(this, Qt::Dialog);
    dialog.setDirectory(QDir::homePath());
    dialog.setFileMode(QFileDialog::ExistingFile);
    dialog.setNameFilter("File iJVM (*.ijvm)");
    if (dialog.exec()) {
        std::string _filePath = dialog.selectedFiles()[0].toStdString();
        const char *filePath =_filePath.c_str();
        this->load_ijvm_prog(filePath);
    }
}

void MainWindow::load_ijvm_prog(const char *filePath) {
    std::cout << filePath << std::endl;
    ijvm_new((char*)filePath);

    //constants
    {
        //cleaning
        int rows = ui->constant_pool_table->rowCount();
        for (int i = 0; i < rows; i++){
            ui->constant_pool_table->removeRow(i);
        }

        //loading
        int cpsize = get_constant_pool_size();
        ui->constant_pool_table->setRowCount(cpsize);
        QStringList headers = QStringList();
        for (int i = 0; i < cpsize; i++) {
            headers.append(QString::fromLatin1(std::to_string(i)));
            int32_t cv = get_constant(i);
            std::stringstream offset;
            offset << std::hex << i;
            std::stringstream hex;
            hex << std::hex << cv;
            ui->constant_pool_table->setItem(i, 0, new QTableWidgetItem("0x" + QString::fromLatin1(offset.str()).toUpper(), 0));
            ui->constant_pool_table->setItem(i, 1, new QTableWidgetItem(QString::fromLatin1(std::to_string(cv)), 0));
            ui->constant_pool_table->setItem(i, 2, new QTableWidgetItem("0x" + QString::fromLatin1(hex.str()).toUpper(), 0));
        }
        ui->constant_pool_table->setVerticalHeaderLabels(headers);
    }
}

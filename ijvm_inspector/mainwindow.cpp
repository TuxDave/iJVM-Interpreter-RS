#include "mainwindow.h"
#include "ui_mainwindow.h"

#include "ijvm_cpp_wrapper.h" //NB: Not available if compile.sh not executed

#include <QFileDialog>
#include <iostream>
#include <string>
#include <cmath>
#include <thread>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    connect(ui->actionOpen, SIGNAL(triggered()), this, SLOT(actionOpen_triggered()));
    connect(ui->runButton, SIGNAL(clicked()), this, SLOT(actionAutoRun_triggered()));
    connect(ui->runTimedButton, SIGNAL(clicked()), this, SLOT(actionTimedRun_triggered()));
    connect(ui->stepForewardButton, SIGNAL(clicked()), this, SLOT(actionStepForward()));
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

    //variables and stack
    this->update_view();
}

void MainWindow::update_view(void) {
    //cleaning
    {
        while(ui->local_variables_table->rowCount() != 0){
            ui->local_variables_table->removeRow(0);
        }
        while(ui->stack_list->rowCount() != 0){
            ui->stack_list->removeRow(0);
        }
    }

    //local vars
    {
        int lvs_size = get_lvs_num();
        QColor row_color = QColor(0,0,0,255);
        QBrush foreground = QBrush();
        foreground.setColor(QColorConstants::Black);
        for (int i = 0; i < lvs_size; i++) {
            int lvsize = get_lv_size(i);
            int lv_base = ui->local_variables_table->rowCount();
            ui->local_variables_table->setRowCount(lv_base + lvsize);
            for (int j = 0; j < lvsize; j++) {
                int32_t value = get_lv_value(i, j);
                std::stringstream disp;
                disp << std::hex << j;
                ui->local_variables_table->setItem(lv_base + j, 0, new QTableWidgetItem("0x" + QString::fromLatin1(disp.str()).toUpper(), 0));
                ui->local_variables_table->setItem(lv_base + j, 1, new QTableWidgetItem(QString::fromLatin1(std::to_string(value)), 0));
                std::stringstream hex;
                hex << std::hex << value;
                ui->local_variables_table->setItem(lv_base + j, 2, new QTableWidgetItem("0x" + QString::fromLatin1(hex.str()).toUpper(), 0));

                auto bgcolor = QBrush(row_color);
                for (int c = 0; c < 3; c++) {
                    auto item = ui->local_variables_table->item(lv_base + j, c);
                    item->setForeground(foreground);
                    item->setBackground(bgcolor);
                }
            }
            row_color.setRed(std::max(row_color.red() + 3, 255));
        }
    }

    //stack memory
    {
        int stacks_size = get_stacks_num();
        QColor row_color = QColor(0,0,0,255);
        QBrush foreground = QBrush();
        foreground.setColor(QColorConstants::Black);
        for (int i = 0; i < stacks_size; i++) {
            int stack_size = get_stack_size(i);
            int stack_base = ui->stack_list->rowCount();
            ui->stack_list->setRowCount(stack_base + stack_size);
            for (int j = 0; j < stack_size; j++) {
                int32_t value = get_stack_value(i, j);
                ui->stack_list->setItem(stack_base + j, 0, new QTableWidgetItem(QString::fromLatin1(std::to_string(value)), 0));
                std::stringstream hex;
                hex << std::hex << value;
                ui->stack_list->setItem(stack_base + j, 1, new QTableWidgetItem("0x" + QString::fromLatin1(hex.str()).toUpper(), 0));

                auto bgcolor = QBrush(row_color);
                for (int c = 0; c < 2; c++) {
                    auto item = ui->stack_list->item(stack_base + j, c);
                    item->setForeground(foreground);
                    item->setBackground(bgcolor);
                }
            }
            row_color.setRed(std::max(row_color.red() + 3, 255));
        }
    }

    //PC e ISTR
    ui->pc_label->setText(QString::fromLatin1(std::to_string(get_pc())));
}

void MainWindow::actionAutoRun_triggered(void) {
    auto_run();
    this->update_view();
}

void MainWindow::actionTimedRun_triggered(){
    timer_on = true;
    this->timed_runner = std::thread([this](){
        while(this->timer_on) {
            step_run();
            //std::this_thread::sleep_for(std::chrono::milliseconds(100));
            this->update_view();
            std::this_thread::sleep_for(std::chrono::seconds(ui->spinBox->value()));
        }
    });
}

void MainWindow::actionStop_triggered(){
    timer_on = false;
    timed_runner.join();
}

void MainWindow::actionStepForward(){
    step_run();
    update_view();
}

#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <thread>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

public slots:
    void actionOpen_triggered(void);
    void actionAutoRun_triggered(void);
    void actionTimedRun_triggered(void);
    void actionStop_triggered(void);
    void actionStepForward(void);

private:
    Ui::MainWindow *ui;
    void load_ijvm_prog(const char*);
    void update_view(void);

    bool timer_on;
    std::thread timed_runner;
};
#endif // MAINWINDOW_H

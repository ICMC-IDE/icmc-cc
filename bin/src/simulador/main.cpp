#include "Model.h"
#include "Controller.h"
#include <iostream>

using namespace std;

int main(int argc, char *argv[])
{	
//*
  // init threads
  g_thread_init(NULL);
  gdk_threads_init();
//*/

	gtk_init(&argc, &argv);

	if(argc != 3)
	{	cout << "ERRO: numero de argumentos invalido" << endl << "Uso: sim arq.mif charmap.mif" << endl;
		return 1;
	}

	Model *m = new Model(argv[1], argv[2]);
	Controller *c = new Controller(m);

//*
  // enter the GTK main loop
  gdk_threads_enter();
  gtk_main();
  gdk_threads_leave();
//*/

//	delete m;
//	delete c;

	return 0;
}


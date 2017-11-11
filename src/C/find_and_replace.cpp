#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

bool
unorderIsPrefix( std::string const& lhs, std::string const& rhs )
{
    return std::equal(
        lhs.begin(),
        lhs.begin() + std::min( lhs.size(), rhs.size() ),
        rhs.begin() );
}

string removeLeadingSpaces(string arg){
  int index = 0;
  for(auto c : arg){
    if(c == ' '){
      index++;
    }else{
      break;
    }
  }

  return arg.substr(index, arg.size()-1);
}

string dropExtension(string arg){
  size_t lastindex = arg.find_last_of(".");
  if(lastindex == string::npos) return arg;
  else
    return arg.substr(0, lastindex); 
}

string findExtension(string arg){
   size_t lastindex = arg.find_last_of(".");
   if(lastindex == string::npos) return "";
   else return arg.substr(lastindex, arg.size() - 1);
}

string filePostfixed(string arg, string postfix){
  return dropExtension(arg) + postfix + findExtension(arg);
}

int main(int argc, char **argv)
{

   string wordToReplace(argv[1]);
   string wordToReplaceWith(argv[2]);
   string postfix(argv[3]);//the output will be redirected to file called:
                          //argv[4+i] + postfix

   for(int i = 4; i < argc; i++){
      ifstream in(argv[i]);

      if (!in)
      {
	 cerr << "Could not open " << argv[i] << "\n";
	 return -1;
      }

      std::streamsize size = in.tellg();

      vector<char> buf(size);


      string line;
      size_t len = wordToReplace.length();
      while (getline(in, line))
      {
	  while (true)
	  {
	      size_t pos = line.find(wordToReplace);
	      bool leading = unorderIsPrefix(removeLeadingSpaces(line),
					     wordToReplace);
	      if (pos != string::npos && leading)
		  line.replace(pos, len, wordToReplaceWith);
	      else 
		  break;
	  }

	  for(int i = 0; i < line.size(); i++){
	    buf.push_back(line[i]);
	  }
	  buf.push_back('\n');
      }



      in.close();

      auto out =  fopen(filePostfixed(argv[i], postfix).c_str(), "wt");

      fwrite(&buf[0], 1, buf.size(), out);

      fclose(out);

      cout << filePostfixed(argv[i], postfix) << " ";
   }

   
    
}

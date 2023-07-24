using Microsoft.States;
using System.Runtime.InteropServices;
using Windows.Win32.Foundation;

// Create the impl and invoke the method.
IState s = new Washington();
BSTR bstr = new BSTR();
unsafe
{
    s.GetFlower(&bstr);
}

string str = bstr.ToString();

Console.WriteLine($"Hello: {str}");

// Implement the Interface
class Washington : IState
{
    public unsafe void GetFlower(BSTR* flower)
    {
        IntPtr inPtr = Marshal.StringToBSTR("MyFlower");
        *flower = (BSTR)inPtr;
    }
}


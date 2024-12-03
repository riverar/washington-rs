using System.Runtime.InteropServices;
using Microsoft.States;
using Windows.Win32.Foundation;

unsafe
{
    // Create an instance and invoke the method
    IState state = new Washington();
    Console.WriteLine($"Washington's state flower is: {state.GetFlower()}");
}


// Implement the interface
internal class Washington : IState
{
    public unsafe BSTR GetFlower()
    {
        return (BSTR)Marshal.StringToBSTR("Pacific Rhododendron");
    }
}

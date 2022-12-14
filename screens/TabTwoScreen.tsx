import { StyleSheet } from 'react-native';

import EditScreenInfo from '../components/EditScreenInfo';
import { Text, View } from '../components/Themed';
import useWalletInfo from "../hooks/useWalletClient";
import {setStringAsync} from "expo-clipboard";
// import Barcode from 'react-native-barcode-expo';
import { GrazProvider, mainnetChains } from "graz";

export default function TabTwoScreen() {
  const walletInfo = useWalletInfo();

  const copyToClipboard = async () => {
    let authURL = walletInfo?.authURL.toString()
    await setStringAsync(authURL ? authURL : "");
  };

  // let barcodeValue = "ticket info";

  return (
      <View style={styles.container}>
        <Text style={styles.title}>Desktop</Text>
        {/*<GrazProvider*/}
        {/*    // optional*/}
        {/*    grazOptions={{*/}
        {/*      defaultChain: mainnetChains.cosmoshub,*/}
        {/*    }}*/}
        {/*>*/}
        {/*  /!*<Wallet />*!/*/}
        {/*</GrazProvider>*/}
        <View style={styles.separator} lightColor="#eee" darkColor="rgba(255,255,255,0.1)" />
        <EditScreenInfo path="/screens/TabTwoScreen.tsx" />
        <Text onPress={copyToClipboard}>{walletInfo?.authURL.toString()}</Text>
        {/*<Barcode value={barcodeValue}  text={"ticket barcode"} onError={console.error}/>*/}
      </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  title: {
    fontSize: 20,
    fontWeight: 'bold',
  },
  separator: {
    marginVertical: 30,
    height: 1,
    width: '80%',
  },
});

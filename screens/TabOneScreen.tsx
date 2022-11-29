import {FlatList, StyleSheet} from 'react-native';

import EditScreenInfo from '../components/EditScreenInfo';
import { Text, View } from '../components/Themed';
import { RootTabScreenProps } from '../types';
import useWalletInfo from "../hooks/useWalletClient";
import {setStringAsync} from "expo-clipboard";

export default function TabOneScreen({ navigation }: RootTabScreenProps<'TabOne'>) {
  const walletInfo = useWalletInfo();
  const account = walletInfo?.accounts[0];

  const copyToClipboard = async () => {
      let authURL = walletInfo?.authURL.toString()
      await setStringAsync(authURL ? authURL : "");
  };

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Mobile</Text>
      <View style={styles.separator} lightColor="#eee" darkColor="rgba(255,255,255,0.1)" />
      <EditScreenInfo path="/screens/TabOneScreen.tsx" />
      <Text>Mnemonic: {walletInfo?.mnemonic}</Text>
      <Text>Account: {account?.address}</Text>

        <Text onPress={copyToClipboard}>{walletInfo?.authURL.toString()}</Text>
        <Text>Grants:</Text>
      <FlatList
          data={walletInfo?.grants}
          keyExtractor={(item, index) => String(index)}
          renderItem={(e) => <Text>{e.item.authorization.typeUrl}</Text>}
      />
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

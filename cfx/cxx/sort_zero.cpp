#include<iostream>
#include<cstdio>
using namespace std;
int a[500010],mp[500010];
int main(){
    int t;
    scanf("%d",&t);
    while(t--){
        int n;
        scanf("%d",&n);
        for(int i=1;i<=n;i++)scanf("%d",&a[i]),mp[i]=i;
        int ans=0;
        for(int i=1,l=1;i<=n;i++){
            if(mp[a[i]]<mp[a[i-1]]){
                while(l<i){
                    if(mp[a[l]])ans++,mp[a[l]]=0;
                    l++;
                }
            }
        }
        printf("%d\n",ans);
    }
}
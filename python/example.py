from discodisco import Server, Parameterizer, Analyzer

disco = Server()
disco.start()
print("started disco")
print(disco)


class CustomAnalyzer(Analyzer):
    pass


class CustomParameterizer(Parameterizer):
    pass


analyzer = CustomAnalyzer()
parameterizer = CustomParameterizer()

disco.stop()
print("stopped disco")

# descriptor = disco.add_analyzer(analyzer)
# descriptor = disco.add_parameterizer(parameterizer)
